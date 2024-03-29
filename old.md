- 1: All state is global and all state is mutable. One of the many problems that can occur as a result is one piece of code changing the something's state to get its own feature to work even though another part of the code needs that state to stay the same. Every part of code ends up fighting over every piece of state

- 2: You often end up in situations where data is nowhere near the code that actually needs to use it

- 3: In basically every large OOP codebase, you'll always find a HUGE pile of abstraction. It's supposed to help developers add to the code, but it also makes the structure of the code basically impossible to change.

- 4: Most of the time this doesn't matter, but OOP does make your code slower. More abstraction means more for the CPU to do, objects are randomly scattered throughout RAM, you can't find areas to optimize because of how everything is spread out everywhere, you can't implement optimizations because that requires reworking the huge stack of abstractions, etc

<br>

## But there's a MUCH worse problem with OOP, and it all comes down to control flow.

When you're making a program, control flow is the only thing that really matters. When a user clicks a button, they don't case how the factory factories work, they just care about the sequence of actions (control flow) that occurs as a result. Likewise, when you debug something, you have to look at the control flow of the code. Any way you look at it, creating, managing, and examining control flow is EXTREMELY important.

OOP says that control flow needs to be split into objects that don't know the control flow of other objects. That is absolutely ridiculous. Control flow NEEDS to be coordinated, and even though you're not supposed to coordinated control flow between objects, that doesn't change that fact that it has to be done.

If you want a look at OOP gone horribly wrong, just look at BeamNG. There's currently a bug where whenever you save a car configuration, it changes the camera angle, switches to windowed mode, goes back to fullscreen, and resets the camera angle. You hopefully know this already, but YOU DON'T NEED TO CHANGE THE CAMERA ANGLE AND SWITCH TO WINDOWED MODE TO SAVE A FILE. The fact that this bug was even possible is completely ridiculous. How on earth is the control flow of saving a configuration linked to the control flow of changing the camera angle and the control flow of switching fullscreen mode??? The only explanation is that the objects that control configurations are entangled with other objects, which are entangled with other objects, and so on until you get to the objects that are entangled with camera angles and window handling

And it's not like they don't know about this or haven't tried to fix it, they obviously have since it used to be worse. When the bug was first introduced, it would go to windowed mode, change the camera angle, go to fullscreen mode, go to windowed mode, change the camera angle, go to fullscreen mode, and repeat until the camera angle is back to where it started. They've made it better, but they can't fix it. They (apparently) don't know how to. Having one feature somehow affect another feature is inevitable when using OOP because of how it basically obfuscated the control flow. Methods from one object call methods from other objects, which call methods from other objects, and because you're not supposed to know the control flow of other objects, there's intentionally no way to know what's going to happen in your code

<br>

### Let's look at the consequences of this:

- 1: Encapsulation completely fails. Control flow HAS TO be defined per-feature, and even though OOP *tries* to stop this, it can't be avoided. In theory you don't know the inner workings of an object you're interacting with, but in reality, you have to look at (and modify) the control flow of other objects in order to get the control flow of your current object to work

- 2: Abstraction completely fails, basically for the same reason as encapsulation. In theory you can just use an object's functions and it'll just work, but thanks to the entangled control flow and global mutable state, functions in OOP codebases basically never "just work"

<br>
<br>
<br>

### So, is OOP always bad?

Well, first we need a more rigorous definition of OOP. For the sake of this section, I'd say that using OOP / using an OOP mindset means using methods attached to objects / structs as apposed to functions that have a vague mental connection to the data. From my experience, I'd say that there is a scenario where an OOP mindset works

Most of OOP's problems stem from control flow between objects. I have countless examples of how control flow between objects has lead to disaster, but I've never had any problems when the control flow only deals with a single object. One great example of this is ".length()" methods. But even with more advanced functions, if it only deals with a single object, having it be a method instead of a stand-alone function doesn't seem to cause any problems

<br>
<br>
<br>

### If OOP is so bad, why is it so widely used?

I'd say the main reason why OOP is widely used is that it's comforting to know that your program is just a bunch of objects that can be independently modified. Comfort might not seem ver important, but it really is. One big example of this is [2kliksphilip](https://www.youtube.com/@2kliksphilip), who refuses to switch away from Fusion 2.5 and learn a "real editor" simply because of the comfort the simpler systems there

For the past few decades, people have only seen 2 options for writing code: OOP objects or haskell-like FP. Newer languages have thankfully shown people that FP doesn't have to be "magic" and "only for programming wizards", but I don't think procedural / FP will ever be able to match the comfort that OOP provides. Whatever is most comforting is likely to stay the most popular, and if that's true, OOP will stay the most popular way to program

Another contributing factor that keeps people in OOP is how programmers tend to be over-excited about new and shiny features, as can be clearly seen in early Rust game engines. One type of "new and shiny features" is design patterns. People learn OOP, see some problems with it, learn exciting design patterns to fix them, end up with even more problems, learn even more design patterns, and so on. Knowing how to "fix" OOP problems with your huge arsenal of design patterns probably makes you feel like a god, and when faced with the choice of switching to FP or learning more design patterns, you choose more design patterns
