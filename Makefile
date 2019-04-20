APPLICATIONS = hello_world_c tiny_basic snake 6502_basic monotron-app monotron-slideshow

all: $(APPLICATIONS)

rebuild: clean all

clean:
	for subdir in $(APPLICATIONS) ; do \
		$(MAKE) -C $${subdir} clean ; \
	done

$(APPLICATIONS):
	$(MAKE) -C $@

.PHONY: all rebuild clean $(APPLICATIONS)
