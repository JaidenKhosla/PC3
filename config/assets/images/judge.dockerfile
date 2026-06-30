FROM alpine:latest

WORKDIR /judge

RUN apk add openjdk17-jre
RUN apk add g++
RUN apk add python3