# Mad ramblings
> It's okay to cry, your tears are justified.
> It is not over, and there is still time.
> Don't let it kill you while others die.
> Letting you waste away could be a crime.

Here is the EVA bible, I'll try to document all of the choices I make.

While researching I tried to look at other projects, such as:
- Miosix
- FreeRTOS
- RTEMS (not really, TODO)
- Zephyr (not really, TODO)
- Linux RT scheduler

## Scheduler
### Main scheduling queues
How to deal with scheduling queues? 

Right now I'm quite set on using locks everywhere, lock-free structures are sexy, but quite impractical unless I design everything around them.

But that leaves us with a problem, what lock should I use?

This deep inside the scheduler only two main choices come to mind:
- IRQ lock: disable/enable all interrupt as a form of lock
- Pause lock: just disable preemption and the scheduler

The first option is used by FreeRTOS, but honestly, I kind of hate it. It has the HUGE advantage of working with interrupt handlers, but is it a good solution? First of all disabling interrupts might be a little more extreme than just pausing the kernel, but that might just be cargo cult programming (I have no substantial claims to back that off). The REAL problem is the gross oversimplification of the nuances of interrupt management exposed by modern IRQ controller, and with the proper APIs (which I don't want to expose at this level) you can actually do much much more than simply disabling all interrupts for the sake of simplicity.

So, for this reason, IRQ locks are banned in the scheduler.

But what now? How do we wake up a thread from an IRQ? We cannot use a pause lock in there because they don't work in IRQs.

So here are the 3 main solutions that I came with
#### Solution 1 - Miosix
I suspect the reason miosix does this is not strictly due to interrupts, as miosix uses IRQ locks, but this could solve my problem. Miosix never removes thread from its ready queues, it just sets a flag telling the scheduler to skip the thread, which can be done atomically (thus without any locks). This has the HUGE disadvantage of needing to keep waiting threads in the ready queues, which makes scheduling O(n) as you always have to traverse all threads, is this really that big of a performance penalty? Probably not, but I don't like it very much.

#### Solution 2 - Unicorn data structure
What if we forget everything I said about lock free data-structures and just make the main scheduler ready queues lock free? There actually are a couple of papers talking about lock-free concurrent doubly-linked linked-list, or double-ended singly-linked linked-list. The problem is that I need to design everything around them, and even then, proving that everything is correct might be really challenging. So this might sound cool, but probably not as good as the next option.

#### Solution 3 - Scheduler defer
Do we really to wake up the thread _instantly_? Maybe we can just push the thread into a specialized lock-free (yes, lock-free, but this time is really simple, so not much of a problem, and even then it's just used in very specialized APIs for IRQs), then request the scheduler to run. If we some random thread was holding the pause, then the scheduler will run as soon as the pause ends, otherwise it will run immediately. The scheduler can then empty those queues for us. I call this "scheduler defer" as you are deferring operations to the scheduler. The cool thing is this is quite flexible and can allow all sorts of cool things, not only in IRQ handlers but practically everywhere, as it doesn't require any lock. For example, want to spawn a thread inside an IRQ? Why not, setup everything and defer the actual spawn to the scheduler. Also, this is very orthogonal to the scheduler design, a sign of a good design IMO.

As you might have guessed, EVA will implement solution 3, is the cleanest and simplest design.

Another fancy trick I've gathered from the Linux RT scheduler is that I can use a bitmask to track which queues are empty and which queues have at least one element. Why is this useful? Well most modern architectures features specialized instructions to get the highest "on" bit, or in our case, the highest priority queue that is not empty.

This basically grants us 32 priority queues for just 1 hardware instruction, neat! Keep in mind that this technique would be quite impractical to realize with both solution 1/2, showing how solution 3 is once again the most flexible.

### Thread suspend/resume
What primitive should EVA expose in order to allow for mutexes/rwlocks and the whole gang to be implemented? 

One critical point is to make sure that EVA has timed wait since the beginning.

Right now the API is dead simple, `resume`/`suspend`, quite self-explanatory. But, is that enough? It might be, the problem is, since this is the same system used by """user-code""" (even if mutexes are technically implemented at an higher level compared to the whole scheduler, they are considered part of the scheduler), what if bad user-code calls `resume` when a mutex is locked? Should the mutex be resilient to this? Is this what we consider spurious wakeups?

I've seen system deal with this by having different systems with different names, all in OR configuration. This leads to funky situations when you can `resume` from a `suspend`, but you `wakeup` from a `wait`. They all do the same thing, but they can be used by different system in order to not interfere with each others.

In practice this should be fairly simple to implement, but is it actually a good solution?

Miosix implements an hybrid of the two, mutexes are resilient to bad user code, but sleeps use a different flag for sleeping. It makes sense actually, sleeping is often used in conjunction with waiting in order to implement timed waits.

(Correction, upon further inspection, timed waits in miosix DO NOT set the sleep flag, and instead only use the wait state, wack, probably a by-product of timed waits being bolted on after?)

But again, is this the right choice? The above solution basically embeds time based suspend in the scheduler. If we instead re-use the same mechanism and make the system resilient to spurious wakeups (or simply ignore the problem, and tell users who use to be very careful about `resume`), we can implement this on-top of the scheduler, cool eh? In general, I like when a design "sits on top of" another one, it makes them independent, changes are easier, code is smaller and in general code is more separated.

### Mutexes and fair blabber
How should the mutexes be implemented? It might seem trivial, but is it? In general I've identified the following design points:

- Priority inversion/inheritance, should we care?
- Is LIFO priority "good enough"? (miosix does this)
- Is respecting thread priorities actually important? If a thread with higher priority came after a lower priority thread should it skip ahead?
- Where do we even start with rw locks?
  - How do we implement LIFO? Should threads be served in full LIFO order? Or should we just deal out the read lock to ALL waiting threads once at least one read lock is served?
  - How do we implement priority? Same as above, but with priority ordering.
- What about threads that can "sneak-in" between a thread unlocking a mutex or rwlock and waking up the next one? Could be simple with a mutex ("owner" solution from miosix), but what about rwlocks?

One possible solution to ALL of this problem is to offer modular policies, not sure what to do? Implement all of them! The user will choose if he needs the extra fairness, or a fast and stupid lock is good enough.

For the "sneak-in" problem, something that `lock_api` calls fairness property (not sure if this term is correct or made up by them), here is a possible solution:

Add a third state, either "locked", "unlocked" or "unlocked but about to be locked by someone that isn't you".

When you come in you can lock only from "unlocked" state, this makes sure that you cannot "get ahead" of other threads waiting which might have just been woken. If you cannot lock, you put yourself in the queue and wait, if you wakeup then YOU are that "someone that isn't you" that should take the lock, because someone woke you and gave you the lock. What about spurious wakeups? They kind of suck to deal with in this configuration, because if you wake up you are not really sure if you are really THE ONE, or just some rando woke you up, so it might need extra data, such as removing you from the queue.

This is nice because it also works on rwlocks, the only difference is that you have two read counters, one for locked readers and the others for "unlocked but about to be locked by someone that isn't you" readers.

### What about linked lists?
Linked lists are nice, because they can be implemented without allocating, critical inside the scheduler.

But how should we implement them?

Here are some ideas:
- FreeRTOS lists: this is wack, the idea is to separate the header (next/prev pointers) from the body (value), of each node, and use a single header (without its value) as the tail and end pointer. This make editing the queue super intuitive, as there is no special handling of starting/ending nodes. BUT implementing this in unsafe rust is a NIGHTMARE, you have to deal with a lot of stuff. Also, the "resting position" (ie queue empty) of the queue is to have the head/tail node be cyclical, or pointing to itself, this makes the structure immovable, which is a big nono for rust. So extra logic is required to "pin" the list, prepare the head/tail node, and "unpin" the list when you have finished and the list is actually empty, this is done by destroying (setting the head/tail to null?) the head/tail node.
- Miosix lists: simple double-linked list, nothing special, using null pointers to signal start/end of the node chain. But how can we detect if a node is inside the list? If both its next/prev pointers are null it means that it is the only node of list, so it cannot be used as a sentinel value for a detached node. Well, we can check that! If the node has one of its pointers null, check if it is the head, otherwise it is not inside the list. We still need specialized code to deal with start/end pointers, but the structure is completely movable.

Also blah blah, ghostcell ties nodes to only one linked list, making insertion/removal safe without keeping track of the belonging linked list, blah blah, rust dark black magic but it should work.

### Safe or unsafe?

The true rustacean in me is calling, everything NEEDS to be safe.

But is that a good idea? Right now I've spent roughly 4 days trying to implement a safe and sound API for linked lists, and not only I failed miserably, but the more I looked at the problem the more I found holes.

So is it worth it to spend extra time making API safe? No. Internal APIs should be safe as possible, but also kept as simple as possible. It makes no sense to spend days making an API safe, with various contorted API decision to make misuse impossible, where upholding unsafe contracts would have been easier and simpler. In the end, upholding simple contracts is simpler than designing a convoluted API and **proving** that it is indeed safe.