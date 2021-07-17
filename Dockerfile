FROM rust

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install \
    cargo-generate \
    fnm

# Install NodeJS via fnm
WORKDIR /root
RUN echo 16.3.0 > .node-version && \
    eval "$(fnm env)" && \
    fnm install && \
    fnm env >> .bashrc

# Install Chrome (for headless testing)
# Taken from: https://hackernoon.com/running-karma-tests-with-headless-chrome-inside-docker-ae4aceb06ed3
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list'
RUN apt-get update && apt-get install -y google-chrome-stable

WORKDIR /app
