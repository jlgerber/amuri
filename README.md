# amuri - asset management uri
This library provides a means to parse a set of asset management uris directly relating to a specific asset model.

The uri is RESTish, deviating a bit as a nod to simplicity. First, lets talk about what isn't there.
we have no authority. We are not going to bake an authority into the uri. Instead, that detail will be a configuration layer responsibility.

And since each scheme represents a single route, we are not going to bother with name levels. 

There are four different schemes, each corresponding with a container type:
- asset
- instance
- render
- plate

The routes are all very similar
```
scheme:LEVEL/CONTAINER NAME/DEPT/SUBCONTEXT/SNAPSHOT_TYPE?version=VERSION#KEY
```
For example:
```
asset://dev01/testcube/model/hi/alembic_model?version=current#main
instance://dev01.rd.9999/testcube1/anim/hi/alembic_cache?version=1#main
```
## Examples
```bash

./amu "asset://ORG3TEST4/testcube/model/md/fbx_model?version=2#main"

amu "instance://ORG3TEST4/testcube1/anim/md/vrayproxy_anim?version=3#main"

```
# Tests
Amuri tests rely on environment variables and should not be run in parallel. Use:

```bash
cargo test -- --test-threads=1
```