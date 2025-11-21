css: /css/main.css
tags: game js webgl

{{../templates/nav.html}}

# Language and framework choice

I've considered a number of different languages to write a game in. From C to
Elm. Certainly C is the most widely used. Elm probably being the least widely
used. Ruby is what I use most these days. So why did I choose JavaScript? Let's
take a brief look at languages I considered and why I ruled them out.

## C/C++

I have fond memories of C/C++. They were the languages I used most when growing up and for the first 2 years of my career. They certainly perform well. However, testing is not as effortless as other languages. After loving static typing I found I could get everything I wanted from automated tests in dynamic languages and a terser syntax to boot. You also have 10 different ways to allocate memory (this may have standardised a fair bit since I last looked). A big thing for me however, is portability. I want a language I can use anywhere without having to recompile or target different platforms. For engines, there are plenty of choices. Unreal, Source, CryEngine to name a few. They don't come cheap though (although Source 2 is free). But you get what you pay for. Feature-wise and quality is topnotch.

## C\#

Memory management is handled. Performance is pretty good. There are some great high-level frameworks such as Unity. However, pricing is an issue. Static typing is still there as well. FP style is possible. But there is that build/run life-cycle and OMG the tools are not built for automation. Testing, CI, fast feedback are my pillars.

## F\#

An interesting language. It's got great potential. There is even a graphics framework for it. Albeit not a 3D one and it still suffers from the lack of tooling I desire. It's good to see .NET Core is trying to improve on this last point.

## Java

I wrote Java professionally for 6 years. Although I learnt a lot about Test Driven Development and many other aspects of programming, I didn't enjoy writing code in Java. Again, static typing and interfaces just bogged me down. I've not seen any 3D engines that blew me away but to be honest I've not looked too closely.

## Haskell, Clojure, other pure functional languages

Lack of libraries/frameworks. Purely functional languages may have bottlenecks in performance (however, I've no references to back this up). Some things are just quicker to develop in a non-functional way and for a purely functional language this would be a barrier.

However, the main reason for me is that I've not got enough experience in these languages. There is a lot to learn and learning a new language is not on that list.

## JavaScript

As with many web developers over the years, my love and hate of JavaScript are of equal measure. However, since ES6 came along it's gotten a lot better. JavaScript is *THE* language of the web. It was widely used when it was a terrible language. Now that it has some great features it's become a pleasure to work with. It's dominance is assured.

It's a truly multi-paradigm language. For games, I've come to realise that this is essential. For me, coming from an OO background and only just starting to embrace functional style, this makes it immediately accessible.

Performance is pretty good these days and will continue to improve. It runs everywhere. Not only that but WebGL is supported in all major browsers and platforms. It has some great free frameworks such as three.js and babylon.JS. Along with some paid alternatives such as PlayCanvas.

For me, one of the most important features is feedback. You can write ES6 code in the browser and have it run. Immediately. No compilation. You don't even need a transpiler (unless you need some crazy ES7 feature but even then you could probably use a polyfill). This, along with functional programming (where applicable) and Test Driven development is how I intend to write a game that will be bug free and allow me to deliver a product to the widest audience with minimum friction.

## BabylonJS

So after settling on JavaScript as the language, the next choice was the framework. I looked at a few options. I wanted something that worked with 3D, was preferably free and that I could develop on where ever I was. I wasn't fussed about an editor. The type of game I'm hoping to create doesn't need one particularly. Three.js is well established but not specifically for games. PlayCanvas is very well polished but it's editor is online and not free. Babylon.JS is completely free, designed for games and appears to be updated regularly. It also has some supporting tools that may come in handy down the line. I liked the community and responsiveness of the developers so my choice was made.

I will try to abstract the framework choice somewhat so that if I decide to switch, I can with reasonable effort, do so.

Over the last few weeks I've been playing around with BabylonJS and bar a few minor hiccups been pretty impressed with how quickly I've been able to get things looking the way want.

My game, will depend quite heavily on a performant physics engine. Next week I'll demo the 2 integrated with BabylonJS, Cannon and Oimo, along with Ammo.
