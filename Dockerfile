FROM rust:slim AS builder
WORKDIR /build

COPY . .

RUN cargo build --release --bin personal-website-backend

RUN cd tools/boris-cli && cargo build --release


FROM julia:1.10-bookworm
WORKDIR /app

RUN apt-get update && apt-get install -y \
	libxt6 \
	libxrender1 \
	libxext6 \
	libgl1-mesa-glx \
	&& rm -rf /var/lib/apt/lists/*

RUN julia -e 'using Pkg; Pkg.add(["CSV", "DataFrames", "Plots"]); Pkg.precompile()'

ENV GWSwstype=100

COPY --from=builder /build/target/release/personal-website-backend /app/server
COPY --from=builder /build/tools/boris-cli/target/release/boris-cli /app/boris-cli

COPY tools/boris-cli/plot.jl /app/plot.jl

EXPOSE 3030

CMD ["./server"]
