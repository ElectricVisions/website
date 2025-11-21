css: /css/main.css
tags: game f# opengl

{{../templates/nav.html}}

# Cube rendering library in F#

I finally reimplemented my basic cube rendering library in F# and OpenGL from JavaScript and WebGL.

![1 million cubes](/images/cube-demo.webp)

My original mission for the JavaScript test was to render 1 million cubes at
60fps. While I'm not running on the best desktop graphics card (I'm on a laptop)
my game idea requires as many cubes on the screen as possible. Optimizations can
be done, however, I wanted to avoid doing that upfront before testing out my ideas.

With the F# version I was expecting it to be fast but I was still impressed.
1 million cubes rendered at 60fps without breaking a sweat. Of course a game
has a lot more to do besides render cubes but it has given me the headroom
I needed.

## What went well?

The fact **I had tests** for the functional modules made converting them so much
easier and gave me a lot of confidence I had implemented them correctly.

Having switched to dynamic languages many years ago it was refreshing to have
the safety of types again. F# really strikes a good balance with its type
inference.

I decided I would not refactor any part of the library during the conversion
process except when the language demanded it. It's really hard to keep to
this when porting code but really helps you focus on completing the conversion.
It's something I learned from a colleague many years ago. Thanks Dan.

## What didn't go well?

The only thing that stopped me from seeing something immediately on the screen
was a typo in my GLox module which handles all the rendering. This is one
of the few untested modules (a conscious decision based on the amount of effort
that would be required to test it). However, referencing the original code, I
was able to spot the mistake and got some cubes rendering.

There were then several issues to resolve:

1. Nothing was moving. The camera was meant to be zooming through the scene.
   This turned out to be a difference in the units for the delta time of
   Silk.NET/OpenGL verses JavaScript's performance timers. It was probably just
   moving *really* slowly.
1. The grid wasn't rendering properly. Missing a lot of the cubes. Simple
   miscalculation of the grid to flat array (`z * x *3` instead of z `*
   gridSize + x * 3`). One time I strayed from my goal of not changing the
   original implementation.
1. Silk.NET has separate rendering and update threads and porting my WebGL
   code only had a single redraw function. This resulted in a weird jerky
   camera movement which smoothed out when moving the mouse (presumably due
   to the render event being called a lot more when handling mouse events).
1. Finally, the normals didn't seem to be applied to the different faces. This
   was in the shader. In WebGL (OpenGL ES) I'd found a pair of shader functions
   that would calculate the normals on the fly (`dFdx`/`dFdy`). However, it
   seems these functions were having no effect in OpenGL. It was possibly due to
   a subtle bug I had introduced but I decided to stick with a default
   implementation and simply pass the normals in to the vertex shader along with
   the vertices.

After figuring out all this I was able to get a proper demo rendering and with
huge performance gains.

## What's next?

As mentioned above, I ported the code directly from JavaScript. Not only that
but it's the first iteration of the library. So I want to run the code through a
few refactorings as follows:

1. Merge the 2 `drawArraysInstanced` calls by having color settings for all cubes
1. Think about the data structure that will be needed to manage the state of all
   the cubes. Currently they're optimised for passing to the GL `DrawArrays*`
   function, however, the physics library will require them in a different
   format.
1. Look at the API to GLox. Refocus it to the task of rendering cubes.
1. Add Text rendering - We'll need this soon to output debug and other info
1. Continue to work on ideas for Matter Shooter, an arcade style platformer with
   some of the concepts I have planned for the full game.

Until next time!
