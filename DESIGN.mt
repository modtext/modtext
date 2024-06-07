# The Design/Architecture of %Text

This document represents an overview for those interested in how %Text works.
You may be interested in using %Text, extending it, or just curious, but this
provides a separate forum than the README, which is intended for any casual
%Text user.

%Text centers around a pulldown parser.  This parser consumes text and outputs a
structured representation.  This core _has no awareness of formatting_
generally.  If we use an analogy to a classical compiler: %Text can be broken
into three "phases" of processing: lexing, parsing, and "compiling"
(/formatting).  These phases may be used indepedently or in concert and are what
provide the modular/extensible power behind the %Text modular text modification
system.
