The Lore language is a set of methods for representing information distribution and detail composition.

First, according to certain configurations, convert `*.lore` into Rust language data objects, and then convert them into specific implementations.

# Lore For Collection

## Syntax

3 syntax are built-in now.

**Atom**

```lore
[ * ]
```

**Link**

```lore
[ * ] = [ * ]
```

**Domain**

```lore
+ [ * ]
```

As the example file indicated

```lore
this is an example file

+ an example domain with some website links
  bai_du = https://www.baidu.com
  zhi_hu = https://www.zhihu.com
  bili_bili = https://www.bilibili.com

+ a domain can be put inside another domain
  + like this
    + and this
      + and more
        ...
```
