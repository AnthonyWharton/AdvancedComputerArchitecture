# Copyright 2018 Anthony Wharton
ifndef VERBOSE
.SILENT:
endif

# Do not run this makefile in parallel - will not apply to recursive makefile
# calls, so running with -jX may still help.
.NOTPARALLEL:

.PHONY: default $(SUBDIRTARGETS) $(ALLTARGETS)
.DEFAULT_GOAL := default

# Targets implemented by all sub-projects
TARGETS       := build disassemble clean
# All subdirectories as a single word (i.e. with no /'s or .'s etc)
SUBDIRS       := $(subst /.,,$(wildcard */.))
# All targets of the form <subdir><target>
SUBDIRTARGETS := $(foreach s,$(SUBDIRS),$(addprefix $(s)\:,$(TARGETS)))
# All targets of the form all<delim><target>
ALLTARGETS    := $(addprefix all\:,$(TARGETS))

# Returns the n'th ($1) item in the : separated string ($2) 
SPLIT = $(shell echo $2 | cut -f$1 -d:)

# Recursively calls make on all subdirectory targets with the given target
$(ALLTARGETS) $(TARGETS):
	$(MAKE) $(addsuffix :$(call SPLIT,2,$@),$(SUBDIRS))

# Recursively calls make on the subdirectory specified with the given target
$(SUBDIRTARGETS):
	echo "================================ $(call SPLIT,1,$@)"
	-$(MAKE) -C $(call SPLIT,1,$@) $(call SPLIT,2,$@)

# Default target, do nothing, run documentation
default:
	echo "No target specified, Aborting."
	echo "Specify target in the form [<all|PROGRAM>:]<TARGET>"
	echo "Where:"
	echo "  PROGRAM is one of: $(SUBDIRS)"
	echo "  TARGET  is one of: $(TARGETS)"

