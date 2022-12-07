.PHONY: all
all:
	@echo "Run my targets individually!"

env/pyvenv.cfg: pyproject.toml
	# Create our Python 3 virtual environment
	rm -rf env
	python3 -m venv env
	./env/bin/python -m pip install --upgrade pip
	./env/bin/python -m pip install -e .[dev]

.PHONY: dev
dev: env/pyvenv.cfg

.PHONY: package
package: env/pyvenv.cfg
	./env/bin/python -m build
