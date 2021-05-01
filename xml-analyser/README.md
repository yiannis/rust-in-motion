# A tool for analysing XML files containing a list of items

## Description

This tool has three main functionalities:
1. Provides an overview of the internal structure of an XML file
2. Extracts specific values or does some logical processing on elements
3. Converts to a smaller XML with different ways to select items

## Command

```
$ analyse-xml <file.xml> <--nesting XPath> <option>
```

## The supported XML file structure

The XML file should contain a list of "items" with very similar contents
Eg:

```xml
<books>
  <book>
    <title>Title 1</title>
    <author>Author 1</author>
    <isbn>0001</isbn>
    <language>UK</language>
  </book>
  <book>
    <title>Title 2</title>
    <author>Author 2</author>
    <isbn>0002</isbn>
    <rating>3</rating>
  </book>
...
</books>
```

In this case, the "item" is the `<book>` element.

## Nesting

We need to specify a nesting for the "items" inside the file. This is done by using an XPath, eg:

```
--nesting /books/book
```

## Options

### Provides an overview of the internal structure of an XML file

* `--analyser`: List all possible tags with their XPath inside the nested item

### Extract specific values or do some logical processing on elements

* `--grep <XPath> [rx]`: Print the value of the element with `<XPath>` [that optionally matches rx] for each item
* `--uniq <XPath>`: Test if all values of element with `<XPath>` for each item are unique inside the XML file

### Convert to a smaller XML with several ways to select items

* `--head <N>`: Print a new XML, containing only the first N items
* `--exrtact <XPAth> <rx> [N]`: Print a new XML with `[N]` items where `<XPath>` value matches `<rx>`
