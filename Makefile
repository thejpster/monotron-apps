.PHONY: all rebuild clean

all:
	$(MAKE) -C hello_world_c all
	$(MAKE) -C tiny_basic all

rebuild:
	$(MAKE) -C hello_world_c rebuild
	$(MAKE) -C tiny_basic rebuild

clean:
	$(MAKE) -C hello_world_c clean
	$(MAKE) -C tiny_basic clean
