##
## EPITECH PROJECT, 2023
## Zappy
## File description:
## Makefile
##

NAME			=	r-type
DEBUG_NAME		=	target/debug/$(NAME)
RELEASE_NAME	=	target/release/$(NAME)

LINUX_NAME		=	pbrain-gomoku-ai

all:
	cargo build --release
	cp $(RELEASE_NAME) $(LINUX_NAME)

clean:
	cargo clean

clippy:
	cargo clippy

debug:
	cargo build

re:	clean all

tests_run:
	cargo test

.PHONY: all clean debug re tests_run