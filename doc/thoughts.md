
# Headers

Headers have lots of drawbacks:
- Make compilation slow
- Have to make sure header and source are consistent
- Make building more annoying

The one advantage is that it make the interface of a file clear.
ie: You can clearly see what the functions are, ignoring definitions.

Although it would be nice to have the public function definition at the top,
it should be fine to simply use the "public" keyword next to functions that
are exported. The user can search for this keyword to see the exported functions.

# Object-oriented programming

## Uses of structs and classes in C++

I use structs for simple objects, where their primary purpose is to hold state.
The "interface" of the struct, is the public member variables.

There are a limited number of member functions, which are utility functions.
If a member function requires state, which is only used for that function, then it should be private.
An example might be get_rotation_matrix(), which returns a stored value if orientation hasn't changed, or recomputes a value.
This would have some private state for the dirty flag and saved value.

I use classes for objects where their primary purpose is to perform a function.
The associated state is only there to help the functions, and shouldn't be important to users of the class.
The "interface" is the ste of public methods.

There might be an occasional part of the state that needs accessing, in which case this is put behind an accessor.
Although accessors are generally superfluous, when using a class which you usually interact with through methods, it is more
consistent to use an accessor.

An example of something that uses a class is a renderer, which provides the functions initialise(...) and render(...).
The alternative would be initialise(render_data, ...), render(render_data, ...).
In this case, passing the data is clunky.

## Inheritance

Viewing inheritance as an "as-is" model of classifying the world is impractical.
This simply overcomplicates things, and can easily lead to a situation where there is no inheritance pattern that gives you
what you want.

Prefer composition over inheritance. Whenever you need flexibility in state and functions, consider an ECS system or equivalent.

Inheritance IS useful when you use a virtual base class to define an interface, from which child classes implement this.
The drawback is you need to use polymorphism. Therefore, this is only really suitable when you only have one object you are using,
or a small number (eg: have a numerical integrator, which has multiple possible implementations).

When you have an array of objects, all as pointers to base classes, this becomes inefficient since you need to jump around all over the place)
(Maybe you could use tagged unions, then cast the data part to the relevant pointer - would keep everything contiguous, and there might
not be too much wasted memory if the objects are of similar sizes).

### Constructors, destructors, RAII

RAII is a good idea.
The only drawbacks I have seen people complain about is the hidden stuff that happens on initialisation, and how it uses dynamic allocation.

Perhaps it would be useful to make it easier to choose an allocator, eg: allocate from a pool of objects instead of the heap, if you are
creating lots of the same object.

Also: you might want to avoid defaut initialisation - only initialise the data when necessary.
Often you might want to allocate memory for something, but leave it unused.
