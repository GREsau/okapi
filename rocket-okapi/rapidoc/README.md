# RapidDoc

This is a project the visualizes the `openapi.json` file into a webpage the user can navigate and
use for searching though the API documentation.

Repo: https://github.com/rapi-doc/RapiDoc
Webpage: https://rapidocweb.com/

## How to update
Download the latest release: https://github.com/rapi-doc/RapiDoc/releases/latest
Copy the following file from the downloaded archive.
All files are located in the folder `/RapiDoc-x.x.x/dist/`:
 - rapidoc-min.js

The file `index.html` is custom for `rocket-okapi` and thus should not be replaced.

The `index.html` and `rocket-okapi/src/rapidoc.rs` should be update to support new tags added.
It should thus be compared with https://rapidocweb.com/api.html

Make sure you also document the update in the changelog.
