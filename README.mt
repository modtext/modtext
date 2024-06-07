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
\*\**bolded*\*\*, \*\*\**/bold and italic/*\*\*\*), %Text dilineates each simple
format type with a unique symbol set.  All symbols may be escaped with a
backslash ("\\").  This also allows for emphasis using formatting not supported
in other systems... like underlining (😱).

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

Headings have a level denoted by the number of prefixed octothorpes.  I.e. "#
Heading" is a first level heading, "## Sub-heading" is a second level, etc. up
through a default styling of six sub-headings (###### Sub-sub-sub-sub-sub-sub
Heading).

## Lists

Lists have a single representation.  The distinction in other systems between
ordered and unordered lists is provided via a symbol-generator concept.
Unordered lists just provide the same symbol for every element, while ordered
lists may either explicitly state the text/symbols used or may use a processor
to replace symbols with a desired (often-related) set as in incrementing numeric
lists.

## Blockquote

TODO

## Comments

So, you want to include information for the editor but not the viewer?  Crazy
thought, isn't it.  Some markup systems might make you think so.  %Text supports
how you want to work.

\; Simple one-line comments are lines prefixed with a semicolon.

\%Comment{
    If you want a block comment, it goes in a mod-block with the Comment
    processor.
}

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

# Peaking behind the curtain

Most editors of %Text documents will only ever interact with the high-level
syntactic sugar provided to achieve tenets #1 and #2.  Tenet #3, however, is
fulfilled by the low-level structural model which provides all the functionality
in a %Text document.

Every piece of formatting sugar desugars to a registered processor with optional
arguments and body.  One can think of any given %Text document having three
"canonical" representations: 1) the human-readable sugarized form, 2) the
tree-structured representation, 3) the expanded "human understandable"
long-form.  This long-form provides a self-consistent mental model and makes it
simpler to debug complex interactions with the processor system.

A simple example of this is to show the long-form of \*\/*/bold and italicized
text/*\/\*.

Short form: \*\/*/bold and italicized text/*\/\*
Long form: %Style[bold]{%Style[italic]{bold and italicized text}}

In this case we see that *bold* and /italics/ are both processed by the `Style`
processor, just with a different parameter.  Note that `bold` and `italic` here,
passed to the `Style` processor are both symbols, not strings.  They have a
unique singleton type which makes processing fast, simple, and allows for
straightfoward error generation.  The single-argument form of the Style
processor takes an enum of bold|italic|underline|strikethrough.
