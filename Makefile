run:
	@make -C kernel run

clean:
	@make -C kernel clean

fmt:
	@cd arch && cargo fmt && cd ..
	@cd drivers && cargo fmt && cd ..
	@cd kernel && cargo fmt && cd ..
	@cd mkfs && cargo fmt && cd ..
	@cd user && cargo fmt && cd ..