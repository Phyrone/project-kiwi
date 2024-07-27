---
title: Glide ID
---

# Glide (ID)
**G**lobal and **L**ocal **I**dentifier with **D**omain **E**ncapsulation.

## Introduction
Glide is a globally unique identifier for Federated systems. It consists of a:
- The **Local** part, which is a 64-bit integer.
- The **Global** part, which is a domain identifier.

It is technically also an address, like an email address or a URL.

In text form, Glide is represented as `Local:Global` where local is prepresented as a base36 string and global is a domain name. glide ids are case-insensitive but are represented in lowercase. 
```glide
base36-string:example.com
```
### Local Part
The local part is a 64-bit integer that is unique within the domain. It is used to identify an entity within the domain. 
Typically, the local part is generated using [snowflake](https://en.wikipedia.org/wiki/Snowflake_ID) or similar algorithms.
Newer objects should have a higher value than older objects.
Comparing objects by age can only be done within the same domain.
As mentioned above, the local part is represented as a lowercase base36 string.
### Global Part
The global part is any valid domain name. 
It is used to identify the organization the entity belongs to.
The domain also is the first beacon to find the entity.
## Purpose
The purpose of Glide is used to find and identify entities in Project Kiwi.
It is used for all or almost all entities in the system.
