APPLICATIONS := $(wildcard examples/*)

all: $(APPLICATIONS)

rebuild: clean all

clean:
	for subdir in $(APPLICATIONS) ; do \
		$(MAKE) -C $${subdir} clean ; \
	done

$(APPLICATIONS):
	$(MAKE) -C $@

.PHONY: all rebuild clean $(APPLICATIONS)
