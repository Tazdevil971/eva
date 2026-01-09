#!/bin/bash

case $1 in
    linux)
        rm .cargo/config.toml
        ln -s config.linux.toml .cargo/config.toml
        ;;

    stm32f767)
        rm .cargo/config.toml
        ln -s config.stm32f767.toml .cargo/config.toml
        ;;

    *)
        echo "Unrecognized target!"
        ;;
esac