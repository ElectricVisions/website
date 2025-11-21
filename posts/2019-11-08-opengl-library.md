css: /css/main.css
tags: game f# opengl

{{../templates/nav.html}}

# OpenGL Library Choice

WebGL interfaces directly with JavaScript, OpenGL is natively C. So we need some bindings in F# or C# to allow us to interface with OpenGL. To add further complications we also need to handle Window initialization which can get pretty tricky when dealing with multiple platforms.

## Requirements

My criteria for my desired library are:

1. Must allow me to interact with OpenGL functions directly to allow me to
   port my current JavaScript library
2. Work with .NET Core 3+
3. Be well maintained
4. Preferably have built-in support for Window creation cross-platform
5. Have some working examples and/or documentation

## The candidates

I couldn't find any mature libraries specifically for F# but, as I understand
interoperating with other .NET languages is straightforward enough in F#.
Although I wasn't expecting much in F# I was surprised to find a lack of modern
libraries in C# but to be fair .NET Core 3 has only just been released.

### OpenGL.NET

This was my first choice when I was initially looking around as it seemed
the closest to OpenGL. However, it's maintained by a single person from what
I can tell and seems to have stagnated somewhat. I also couldn't get the
examples to run under .NET Core.

### OpenTK

I discounted this one at first as I was under the impression it provided a
higher level interface and although the windowing parts are pretty high
level, it does have OpenGL bindings. Again, though I struggled to get the
examples to run under .NET Core. There is a new version that is nearing
completion but then I (re)discovered...

### Silk.NET

This library is still under development but it's moving fast and
some of it's developers are from OpenTK. It seems modern and it's the only
library whose example ran under .NET Core out of the box for me. Support
and comms also seem top notch and the library is at Preview 3 stage as of
writing this.

The only step not mentioned in the README is copying of the
[GLFW](https://www.glfw.org/) DLL to the project directory. However,
it mentions this when it fails to run.

So I decided to try integrating Silk.NET with F#.

## Trying out Silk.NET

Checking their [announcements channel on discord](https://discord.gg/VkYSmgQ)
they have a number of Nuget packages available.

I decided to test the library by converting the existing C# examples from Silk.NET
GitHub repo into F#. See [my repo](https://github.com/PhilT/silk.net-examples-fsharp)
for the full source code.

Here, I'll note down the steps I took and highlight a few issues I had
on the way.

### Creating the projects

I use Paket to manage my dependencies. I'm used to Node's NPM and Ruby's
Rubygems so Paket felt natural. However, I've found it to be a little
tougher to get started with, mainly due to the multiple ways in which
it can be used. Fortunately, if you stick with .NET Core 3 it's
relatively straightforward.

I started by using F# Interactive, however, I quickly ran into an issue
with indirect dependencies not loading. There are some outstanding issues
with .NET Core to fix this. After hacking together a workaround I eventually
abandoned FSI and went with building a project as I'd need to do this
anyway.

    mkdir silk.net-examples-fsharp
    cd silk.net-examples-fsharp

### Blank Window example nailed

We just need the Windowing package for this one.

    mkdir blank_window
    cd blank_window
    dotnet new console -lang f#
    dotnet add package Silk.NET.Windowing -v 1.0.0-preview3

It went well. F# automatically handles `Action` delegates if you pass them
though a lambda.

    window.add_Move (fun position -> printfn "Moving %A" position)

### Fun with pointers in the Triangle example

One little F# feature that tripped up a newbie like me was that declaring
a function without any parameters is actually declaring a value which gets
immediately evaluated. `load` was being evaluated before `Window.Create(...)`
was called. Adding an empty tuple (`let load () = ...`) fixed the problem.

Another issue I ran into on this example was how to pass a void pointer
to a C# method. Getting a pointer to an array is straightforward with the
`fixed` keyword. However, this is typed to the original array type.

    use floatPtr = fixed vertices

This is where I wish the documentation for F# Core library was more current
(ticket raised in docs.microsoft.com). It turns out the `toVoidPtr`
function was added to F# 4.5 ([See details](https://github.com/fsharp/fslang-design/blob/master/FSharp-4.5/FS-1053-span.md)).
I was then able to add:

    let voidPtr = floatPtr |> NativePtr.toVoidPtr
    gl.BufferData(GLEnum.ArrayBuffer, size, voidPtr, GLEnum.StaticDraw)

Another thing that stumped me here was once I got the Triangle to display. It
wasn't displaying correctly. The left side looked correct but the right side
was flat against the right edge of the screen. This turned out to be the `float`
size in F#. The default is 64-bit. So my conversion from C# `sizeof(float)` to
F# `sizeof<float>` should have been `sizeof<float32>`. F# type inference
couldn't help me here as the points had, as previously mentioned, been
converted to a void pointer.

### Better handling of Action delegates in the Input Test example

This one was easier to convert as I was getting the hang of things. One thing
that looked a bit ugly to me though was having to wrap all the Action delegates
in lambdas, especially when you want to use the function more than once.
Fortunately, it's simple to move the lambda to the function declaration itself.

    let mouseOnMouseUp = Action<IMouse, MouseButton>(fun m b ->
      ...
    )

Then it can be added to a listener like this:

    mouse.add_MouseUp(mouseOnMouseUp)

