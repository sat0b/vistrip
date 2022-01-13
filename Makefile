
qdrant/serve:
	cd qdrant && cargo run --bin qdrant

qdrant/serve/grpc:
	cargo run --bin qdrant --features=grpc

qdrant/kill:
	lsof -i:6333 -i:6334 | awk 'NR>1{print $2}' | sort | uniq | xargs -I@ kill -9 @

qdrant/clean: qdrant/kill
	mv storage /tmp
