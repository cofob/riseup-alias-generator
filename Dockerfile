# Export pip dependencies
FROM python:3.11-slim AS poetry

RUN pip install poetry
COPY poetry.lock pyproject.toml /
RUN poetry export -f requirements.txt -o /requirements.txt

# Final image
FROM python:3.11-slim AS final

WORKDIR /app

COPY --from=poetry /requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
COPY riseup_alias_generator /app/riseup_alias_generator

CMD ["python", "-m", "riseup_alias_generator"]
