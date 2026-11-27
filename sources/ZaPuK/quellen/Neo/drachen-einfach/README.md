# drachen-einfach

This software implements a mock/showcase online service to create example cases based on the [model service description for commercial dragon keeping (DE)](https://docs.gov.de/fim/assets/files/20211102_Musterleistungsbeschreibung_Drachenhaltung-4cd2b68656c62b94cfbbe27b67c3a0bc.pdf) in the context of the ["Neo" pilot project](https://gitlab.opencode.de/fitko/matrix-g2x) for Matrix-based communication between the government and private individuals.

You can find an instance deployed at https://drachen-einfach.fitko.dev

## Fork tree
```
Musterformular
`- prototypes/bundesident-showcase-onlinedienst
   `- drachen-einfach.de
```

## How to run
```bash
poetry install
export FLASK_APP=server
poetry run python -m flask run
```
