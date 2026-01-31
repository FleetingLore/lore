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

## How to use

Download `lfc.exe` on your Windows pc. Optionally you can try to test the example file.

<img width="1024" height="729" alt="image" src="https://github.com/user-attachments/assets/c1e7605d-5c0b-4e62-89fe-c5f4f6a6d7b8" />

Open your terminal software and type in this format

```ps1
PS ...> ./lfc.exe test.lore test.html
```

Then display the target with an HTML page tool.

<img width="1023" height="619" alt="image" src="https://github.com/user-attachments/assets/4700cd8d-9e03-4fe2-9cbe-c1e1bffb9f31" />

I'm getting to make it better.
