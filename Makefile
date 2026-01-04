ifeq (revision,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

.PHONY:

postgres_db:
	docker run --name=blog_db \
	 			-e SSL_MODE='disable'\
				-e POSTGRES_USER=postgres\
				-e POSTGRES_PASSWORD=postgres\
				-e POSTGRES_DB=blog_db\
				-e TZ=GMT-3\
				-p 5438:5432 -d --rm postgres:17.0-alpine3.19
