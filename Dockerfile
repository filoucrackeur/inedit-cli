FROM scratch

COPY --chmod=755 binary/amd64/x86_64-unknown-linux-gnu/release/inedit-cli /inedit-cli

CMD ["/inedit-cli"]
