updated: 2021-02-15
tags: game f# .net


# Project Reboot - Going all in on F#

Whoah! Over a year since I posted updates! Selling our house and all our stuff and moving into a van is time consuming! In that time I've had a chance to re-evaluate some of the choices I made when starting this project.

After an encouraging start with my library in JavaScript I got the nagging
feeling the performance still wasn't quite there for the number of cubes I
wanted to work with. Sure, there are optimisations that can be done but I
didn't want to have to deal with that from the start. The final straw came
when I realised that real-time ray tracing would be a great addition to my game.
If I wanted the latest and greatest I'd need to use a compiled language.

I previously looked at F# as an option but discounted it due to it's reliance
on Visual Studio and the command-line tooling being somewhat immature along
with having to learn another language. However, with the potential performance
issues I was facing with JavaScript, I took another look. .NET Core 3 has just
been released which adds F# Interactive and the tooling generally seems to be
improving.

I decided to take the plunge and over the last few months I've been slowly
getting up to speed with the language. There is also the extra hurdle of
learning .NET but this opens up a bunch of useful libraries which I'm now ready
to explore.

I'm sticking with low-level OpenGL as that should allow me to port over what
I've already developed in JavaScript.

As part of learning F# and some associated tech which I may need I've
rewritten this blog to generate HTML using F# and some libraries like
[FSharp.Literate](https://fsprojects.github.io/FSharp.Formatting/literate.html)
to ensure code in articles actually runs correctly.
