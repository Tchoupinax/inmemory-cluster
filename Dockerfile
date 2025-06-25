FROM scratch

WORKDIR /app

COPY target/release/inmemory-cluster /app/inmemory-cluster

CMD /app/inmemory-cluster
