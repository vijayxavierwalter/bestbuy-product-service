# Best Buy Product Service

## Overview

The **Product Service** is a microservice in the Best Buy Cloud-Native Application.
It provides a REST API to manage and retrieve product information for the store.

This service is used by:

* **Store-Front** → to display products to customers
* **Store-Admin** → to manage product inventory

---

## Responsibilities

* Retrieve all products
* Retrieve a single product by ID
* Add new products
* Update existing products
* Delete products

---

## API Endpoints

| Method | Endpoint        | Description          |
| ------ | --------------- | -------------------- |
| GET    | `/`             | Get all products     |
| GET    | `/{product_id}` | Get a single product |
| POST   | `/`             | Add a new product    |
| PUT    | `/`             | Update a product     |
| DELETE | `/{product_id}` | Delete a product     |
| GET    | `/health`       | Health check         |

---

## Sample Product Data

This service currently uses in-memory product data representing a Best Buy electronics catalog, including:

* Laptops
* Monitors
* Headphones
* Smart TVs
* Accessories

---

## Tech Stack

* **Rust**
* **Actix Web**
* **Docker**

---

## Project Structure

```text
src/
├── main.rs
├── startup.rs
├── routes/
├── model/
├── data/
├── configuration/
```

---

## Running Locally

### Prerequisites

* Rust installed
* Cargo package manager

### Run the service

```bash
cargo run
```

The service will start on:

```
http://localhost:3000
```

---

## Docker

### Build image

```bash
docker build -t yourdockerhub/bestbuy-product-service .
```

### Run container

```bash
docker run -p 3000:3000 yourdockerhub/bestbuy-product-service
```

---

## Deployment

This service is:

* Containerized using Docker
* Deployed to **Azure Kubernetes Service (AKS)** using Kubernetes manifests
* Integrated into a microservices architecture with other services

---

## Related Services

* **store-front** – Customer web application
* **store-admin** – Admin dashboard
* **order-service** – Handles order processing
* **makeline-service** – Processes order status

---

## Notes

This service was adapted from a lab starter project and customized for the Best Buy Cloud-Native Application final project.
