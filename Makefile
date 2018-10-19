.PHONY: all rebuild clean

APPLICATIONS = hello_world_c tiny_basic slide_show snake 6502_basic monotron-app

all:
	for subdir in $(APPLICATIONS) ; do \
		$(MAKE) -C $${subdir} all ; \
	done

rebuild:
	for subdir in $(APPLICATIONS) ; do \
		$(MAKE) -C $${subdir} rebuild ; \
	done

clean:
	for subdir in $(APPLICATIONS) ; do \
		$(MAKE) -C $${subdir} clean ; \
	done
