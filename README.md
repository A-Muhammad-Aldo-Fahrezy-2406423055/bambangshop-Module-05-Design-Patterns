# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
In Java, interfaces are necessary to ensure the publisher only knows about the update method without knowing the concrete class, enforcing loose coupling. In Rust, traits serve the same purpose. However, in this specific BambangShop case where we only have one type of subscriber, using a single Subscriber model struct is perfectly sufficient and avoids over-engineering. If we had multiple different ways to notify subscribers, a trait would be essential.

Since ID and URL are unique, using a Vec means checking for duplicates or finding a specific subscriber to remove would take linear time. Using a DashMap allows for constant time lookups, insertions, and deletions using the URL as the key. This is much more efficient and necessary for handling many subscribers concurrently.

The Singleton pattern ensures only one instance of an object exists. However, in a multi-threaded web server like Rocket, multiple request handler threads will try to access and mutate this single instance simultaneously. A normal Singleton in Rust still requires synchronization primitives like Mutex or RwLock to be thread-safe. DashMap already implements these concurrent access safeguards internally, so we still need DashMap or a thread-safe wrapper even if we use a Singleton pattern.

#### Reflection Publisher-2
In MVC, if the Model handles data structuring, business logic, and database access, it violates the Single Responsibility Principle and becomes a "God Object." Separating them ensures the Model only defines the data structure, the Repository handles database operations, and the Service handles the core business logic. This makes the code modular, easier to test, and easier to maintain.

If we only use Models, the code complexity within Program, Subscriber, and Notification would explode as they try to handle HTTP request parsing, payload formatting, database queries, and business rules all within a single struct. Modifying the database logic would risk breaking the business logic, and testing them in isolation would become impossible.

Postman has been extremely useful for testing our API endpoints without needing a frontend client. I am interested in using its automated testing feature and collections runner to run integration tests across multiple endpoints sequentially, which will be highly beneficial for ensuring reliability in the Group Project.

#### Reflection Publisher-3
In this tutorial, we use the Push model. The publisher sends the notification payload directly to the subscribers through the HTTP POST request whenever an event occurs.

If we used the Pull model, the advantage is that subscribers only fetch data when they need it, reducing payload size on the initial notification. The disadvantage is that it requires two network requests, which causes higher latency and potential bottlenecks if many subscribers pull data at once.

If we didn't use multi-threading, the publisher would have to send HTTP requests to each subscriber sequentially in a loop. If there are hundreds of subscribers or if some subscribers have slow network responses, the original request would be blocked and take a very long time to return a response to the client. Multi-threading allows the main response to return instantly while notifications are sent in the background.
