.PHONY: all rebuild clean

all:
	$(MAKE) -C hello_world_c all

rebuild:
	$(MAKE) -C hello_world_c rebuild

clean:
	$(MAKE) -C hello_world_c clean
