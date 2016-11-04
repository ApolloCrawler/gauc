FROM buildpack-deps:jessie
MAINTAINER Tomas Korcak <korczis@gmail.com>

EXPOSE 5000 5000

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    cmake \
    gdb \
    openssl \
  && rm -rf /var/lib/apt/lists/*

ENV RUST_CHANNEL nightly

RUN curl -s https://static.rust-lang.org/rustup.sh \
  | sh -s -- --yes --disable-sudo --channel=$RUST_CHANNEL \
  && rustc --version && cargo --version

ENV CARGO_HOME /cargo
ENV SRC_PATH /src

RUN mkdir -p "$CARGO_HOME" "$SRC_PATH"
WORKDIR $SRC_PATH

RUN git clone git://github.com/couchbase/libcouchbase.git && \
   cd libcouchbase && \
   git checkout 2.5.8 && \
   mkdir build && \
   cd build && \
   ../cmake/configure && \
   make && \
   make install && \
   ldconfig -vvv && \
   cd .. && \
   rm -rf libcouchbase

ADD . .

RUN make build-release && \
    cargo install --force --path .

ENTRYPOINT ["/cargo/bin/gauc"]
