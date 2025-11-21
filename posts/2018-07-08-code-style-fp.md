css: /css/main.css
tags: game js fp

{{../templates/nav.html}}

# Code Style and Functional Programming

In this article I discuss my choice of coding style and also touch on functional programming. It's something I've been studying for a while and I'm slowly trying to introduce into my work. Both these things will affect the style of code I post on this blog.

## Code Style

Over the years I've learnt that being consistent is more important than being right. As you probably know, when it comes to code style there is no real right anyway. However, I find it best to follow whatever style is used in the codebase you're working on.

Of course, when working on your own projects you have a bit more freedom. Even better, there are lots of pre-rolled options out there so you don't have to build your own. When choosing one, I find it useful to stick with the presets rather than try to customise things too much. That way if your team becomes more than one you've got a standard style guide to reference.

When working with JavaScript, I've chosen to follow Standard JS. Briefly, you get:

* 2 space indent
* Single quotes
* No semicolons (The linter will let you know where they are needed)
* Always use === (if you're not you should read up on this)
* Catches unused variables (very useful)
* And tons more...

As a Ruby developer the first 3 points here fit naturally. For the semicolon lovers, you should really know where and why they are needed. I find putting them everywhere for the sake of it somewhat sloppy.

## Functional Programming

Why functional programming for games, you ask? Here are a few reasons:

* to write code with as few bugs as possible
* to add code incrementally without fear of regression
* to code as efficiently as possible
* to assert an output for a given input making testing a breeze
* to use all the cool new ES6 features

In order to interface with the Object Oriented BabylonJS library and to take a pragmatic approach to FP, I'm adopting [Functional Core, Imperative Shell](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell).

To aid my learning, I'm currently reading (a very good book so far) Functional-Light JavaScript by Kyle Simpson.

### Point-free style (or more formally, tacit programming)

One term that stopped me in my tracks was point-free style. I've seen this term used before but never quite understood it. So, this time I decided to do some digging and try to put it into my own words to makes some sense of it:

In mathematics, points are the named parameters in an expression. Point-free style, then, removes the explicit use of these parameters, or points.

### An example

We'll use functional programming and point-free style to solve a common problem. We have an array of strings that need to be converted to numbers.

I'd normally write that like this:

    ['1', '2', '3'].map(n => parseInt(n))

Pretty concise, however the `n` in `parseInt(n)` is the point we want to remove. So, I thought, why not just pass the `parseInt` function straight to `map`?

    ['1', '2', '3'].map(parseInt) // [ 1, NaN, NaN ]

Although this is now point-free, `map` passes an index as the second argument to `parseInt` which is the radix parameter.

<aside>
Notice the use of the words <em>argument</em> and <em>parameter</em>. Something I also learnt from Kyle's book. Arguments are what you pass to a function and parameters are the names of the variables assigned to them in the function. I'd never thought of them as distinct before. This is useful.
</aside>

To fix this we can create a little helper function.

    const unary = fn => arg => fn(arg)

This function takes a function as it's parameter and returns another function that calls our passed in function parameter with the single argument.

If ES6 arrow functions are not your thing:

    function unary (fn) {
      return function (arg) {
        return fn(arg)
      }
    }

Now we can do the following:

    ['1', '2', '3'].map(unary(parseInt)) // [ 1, 2, 3 ]

No more points! The only parameters here are functions.

More comprehensive versions of the `unary` function we defined here can be found in functional libraries such as [Ramda](https://ramdajs.com/).

There are a lot of functional programming techniques used here. Passing in `parseInt` as a parameter is called higher-order functions; the use of `map` instead of a for-loop is called declarative programming; and `parseInt` is now point-free.

This example was mostly taken from Functional-Light JavaScript. If you're struggling to get to grips with functional programming I highly recommend it.

Get it in a range of formats at [Leanpub](https://leanpub.com/fljs) or the Kindle edition is available at [Amazon](https://www.amazon.co.uk/gp/product/B0787DBFKH/ref=as_li_tl?ie=UTF8&camp=1634&creative=6738&creativeASIN=B0787DBFKH&linkCode=as2&tag=gamesmatterco-21&linkId=1259904f1ca5dedc5369d70da2ee21a5).

I'll probably post more on FP in the future which I hope will help me better understand and solidify the concepts. In future posts I hope to apply FP to game programming problems.

