FROM ubuntu:24.04

# Install Deno and build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    python3 \
    make \
    g++ \
    unzip \
    && curl -fsSL https://deno.land/install.sh | sh \
    && rm -rf /var/lib/apt/lists/*

ENV DENO_DIR="/root/.deno"
ENV PATH="/root/.deno/bin:$PATH"

WORKDIR /app

# Copy package files and lockfile
COPY package.json deno.lock ./

# Install dependencies
RUN deno install --node-modules-dir

# Copy the rest of the app
COPY . .

# Build the app
RUN deno task build

# Expose port
EXPOSE 3000

# Set environment variables
ENV NODE_ENV=production
ENV PORT=3000
ENV HOST=0.0.0.0

# Run the app
CMD ["node", "build"]
