# PHP adding lib

Library that adds adding functions for php.

### How to install

Run following commands and module file will be generated and moved to php modules directory

```cargo build --release```

```phpize```

```make```

```make install```

After you need to open your ```php.ini``` file and add extension there:

```extension=/path/to/php/modules/php_add.so```

### Doc

Right now only available function is:

```add(x,y, return_type=ADD_RETURN_TYPE_DOUBLE)```

Where ```x``` and ```y``` are either ```double``` or ```int```, ```return_type``` is optional.

For return type you have two possible constants:

```ADD_RETURN_TYPE_INT```

```ADD_RETURN_TYPE_DOUBLE```

```ADD_RETURN_TYPE_DOUBLE``` using by default, or when argument is invalid.

```ADD_RETURN_TYPE_INT``` rounding double results. down ```<=0.5``` and up ```>0.5```

## Examples

You can test module functionality simply by using ```php -a``` in your terminal:

```php -a```

```echo add_double(2.0, 2.0);```