FROM rust:1.67

WORKDIR /usr/src/msdf-atlas-sys
COPY . .

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y cmake clang

RUN cargo build -r