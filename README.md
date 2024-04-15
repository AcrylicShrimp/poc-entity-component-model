# poc-entity-component-model

This repository showcases an implementation of an object (entity) management system in Rust. This system draws inspiration from the Entity-Component-System (ECS) model but is simplified for ease of understanding and use.

Initially, this project aimed to replicate a Unity-like object-component system in Rust. However, Rust's stringent memory safety features limit the flexibility seen in Unity's `MonoBehaviour`, where logic and data are often intermingled freely. In Unity, `MonoBehaviour` uses non-direct references (essentially pointers), which are abstracted to appear seamless to the user. Rust, however, requires a different approach due to its ownership rules, leading us to separate logic and data explicitly.

## Detailed Overview

This model comprises four primary components:

- **Object**: The fundamental unit in this system, each Object owns a list of `Component`s.
- **Component**: Represents a data unit that can be associated with an Object. Multiple Components can be associated with a single Object.
- **Controller**: A unique combination of data and logic that defines custom behaviors. It interacts with the system through lifecycle hooks and event responses.
- **Event**: User-defined signals that Controllers emit to interact indirectly.

The `Controller` plays a pivotal role, holding the user-defined logic. It interacts with the entire system via a context handle, enabling functionalities like adding new Objects, attaching Controllers to Objects, or accessing other Objects and Components. Direct interaction between Controllers is restricted by Rust's safety guarantees, which is circumvented using `Events` for communication.

## Managing Race Conditions

To prevent race conditions, a Controller can manipulate any part of the system except other Controllers. This model maintains a record of actions initiated by a Controller on the context, which are processed once the Controller's reference is released. This approach ensures actions are sequentially and safely executed, respecting Rust's ownership and borrowing rules. Note that some actions are performed immediately, especially manipulating Objects and Components.

## Performance Considerations

While this implementation is not optimized for large-scale productions as it stands, the model could be adapted for use in many real-world game development scenarios. Further optimization and refinement might be needed for handling more complex or performance-intensive applications.
