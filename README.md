# doc-gen

Extract documentation from C source files to generate formatted HTML documentation.

Add the following parameters in a block comment above a function definition:

* `@brief` or `@note` - Description of the function
* `@param <param>` - Description of the parameter `param`
* `@ret`/`@return`/`@retval` - Description of the return value of the function
* `@related` - List related function names separated by a space

For example:



```c
/*
@brief Fetch the value from **map** corresponding to **key**
@param map map to fetch from
@param key to pull value from
@related map_vals map_keys map_add
*/
void* map_get(map_t *map, const char *key);
```
