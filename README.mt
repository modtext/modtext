# The %Text (ModText) Modular Text Modification System

%Text (pronounce "mod text") is a text-based markup system.  The tenets of %Text
are as follows:

1) Be readable
2) Be intuitive
3) Be extensible

In that order.

The %Text name represents two things: a modular text modification system and a
default set of modules which define a markup language.  This file was written in
the %Text markup system.  It is theoretically possible to implement MarkDown or
other markup systems /using/ %Text.

%Text markup is a bit of throwback to old "colloquial" bulletin board (BB)
markup systems.  Unlike the hard to remember and visually indistinct nesting of
the same symbols in systems like MarkDown (i.e. \*/italicized/\*,
\*\**bolded*\*\*, \*\*\**/bold and italic/*\*\*\*), %Text dilineates each format
type with a unique symbol set.  All symbols may be escaped with a backslash
("\\").  This also allows for emphasis using formatting not supported in other
systems... like underlining.

## Standard text formatting:
\\) Italics: \//italicize text by surrounding it in forward slashes.  This
visually represents the right-ward tilt of the text under format./\/
\*) Bold: \**bold text by surrounding it in asterisks.  This is meant to add
weight to the text.*\*
\_) Underline: \__underline text by surrounding it in underscores.  This
visually hints to the line under the text being marked up._\_
\~) Strikethrough: \~~strikethrough text by surrounding it in tildes.  This
visually hints to the line through the text being marked up.~\~

This means that when you have text that's bolded and italicized, you /see/ both
those annotations in the markup.

## Headings

TODO

## Lists

TODO

## Blockquote

TODO

## Literal/Processed Blocks

TODO

`Monospace literal default processor`

```
Monospace literal default processed block
```

`[processor] some text to process w/ "processor"`

```processor
```

## Links

TODO

## Escaping text for future processing

When processing text in a pipeline such as that used for the book publication
process or academic paper submission, it can be desirable to side-step the basic
text markup pass for sections of input.  This is provided by using the default
*`escape`* processor.  The resulting formatted block will be left be and
explicitly annotated as having been escaped.

# FAQ

