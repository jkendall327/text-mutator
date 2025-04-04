### 1. Overview

This is a public-facing web application designed to help users spot small, deliberate errors ("mutations") in a given text. The goal is to enhance proofreading effectiveness by disrupting text familiarity. It serves as a personal learning project focused on React, Rust, and full-stack deployment on Azure.

---

### 2. Core Features

-   **User Input**:
    -   A primary text area for user input.
    -   Supports multiline text and special characters.
    -   **Constraint**: Maximum input length of 5000 characters (enforced client-side and server-side).

-   **Mutation Logic (Backend - Rust)**:
    -   **Goal**: Introduce subtle errors into the input text based on user configuration.
    -   **Mutation Types**:
        -   `SwapLetters`: Swap two adjacent alphabetic characters.
        -   `RemovePunctuation`: Remove a single punctuation character (`.`, `,`, `!`, `?`, `;`, `:`, `-`, `'`, `"`).
        -   `ReplaceHomophone`: Replace a word with one of its homophones (using a predefined internal set).
    -   **Configuration Options (via API Request)**:
        -   `mutation_rate: f32` (Target rate, e.g., 0.1 means aim for mutations in ~10% of sentences).
        -   Toggles (boolean flags) for enabling each mutation type: `allow_swaps`, `allow_punctuation_removal`, `allow_homophones`.
    -   **Mutation Strategy**:
        -   Identify sentences (naive split by `.`, `!`, `?`).
        -   Identify *all possible* mutations within the text according to enabled types.
        -   **Constraint**: Apply *at most one* mutation per identified sentence.
        -   Randomly select sentences to mutate, up to the target number implied by `mutation_rate`.
        -   Within a selected sentence, randomly choose one *possible* mutation of an enabled type to apply.
        -   The final number of mutations may be lower than the target rate suggests, depending on sentence count and available mutation opportunities.
    -   **Backend API Response**:
        -   Endpoint: `/api/mutate` (POST)
        -   Request Body: `{ "text": "...", "config": { "mutation_rate": 0.1, "allow_swaps": true, ... } }`
        -   Success Response (200 OK): `MutationResponse` JSON object.
            ```json
            {
              "mutatedText": "The mutated stirng goes here...",
              "mutations": [
                {
                  "start": 15, // Character index in mutatedText
                  "end": 21,   // Character index in mutatedText (exclusive)
                  "type": "REPLACE_HOMOPHONE"
                },
                {
                  "start": 5,
                  "end": 7,
                  "type": "SWAP_LETTERS"
                }
                // ... other mutations
              ]
            }
            ```
        -   `MutationType` Enum (String values): `SWAP_LETTERS`, `REMOVE_PUNCTUATION`, `REPLACE_HOMOPHONE`.
        -   `start`, `end`: **Character indices** (0-based) within the `mutatedText` string, defining the span of the applied mutation. For single-character changes (removal, swap), `end` will be `start + 1` or `start + 2` respectively *in the mutated string*. For homophones, it spans the replaced word.
        -   **Note**: The Rust `mutator.rs` code needs to be updated to gather and return this detailed `Mutation` information, not just the final string and count.

-   **Mutation Settings Panel**:
    -   Flyout or collapsible UI section.
    -   Controls: Slider for `mutation_rate` (e.g., 0.0 to 0.5), checkboxes for `allow_swaps`, `allow_punctuation_removal`, `allow_homophones`.
    -   Applies sensible defaults on initial load (e.g., rate=0.1, all types enabled).

-   **Text Display & Interaction**:
    -   **Mutated Text Display**: The `mutatedText` is displayed in a main view area. Spans or similar elements might be used internally for later highlighting but are not initially visible or interactive.
    -   **Manual Counting**: A button labeled "Found One!" (or similar) with an associated counter display (e.g., "Found: 0 / 5"). Clicking the button increments the counter. This allows users to self-track their findings.
    -   **Reveal Mechanism**: A button labeled "Reveal Mutations".
    -   **Highlighting**: When "Reveal Mutations" is clicked, the application uses the `start`, `end`, and `type` data from the `mutations` array (received from the backend) to visually highlight the mutated segments within the `mutatedText` display (e.g., different background colors per type).
    -   **Success State**: When the user's manual count equals the actual number of mutations (derived from the length of the `mutations` array), a success message/notification is displayed. Clicking "Reveal Mutations" also shows highlights in this state.

-   **Original Text View**:
    -   The original input text is displayed *below* the mutated text.
    -   Initially styled to be less prominent (e.g., greyed out, slightly blurred).
    -   Becomes fully visible on hover (desktop) or tap (mobile).

-   **Mutation UI Behavior**:
    -   **Typewriter Effect**: When new mutated text is received from the backend, it appears character-by-character in the display area (purely a frontend visual effect).
    -   **Regeneration**: Clicking the main "Mutate" button (which triggers the backend call) clears any previous mutated result, highlights, and counter state, then generates a new mutation based on the current input text and settings.

-   **Copy Functionality**:
    -   A button to copy the *mutated text* (`mutatedText` field) to the user's clipboard.

-   **Error Handling**:
    -   An inline error block/area is displayed when necessary.
    -   **Client-Side**: Message for "Input text exceeds 5000 characters."
    -   **Backend Communication**:
        -   If API returns 4xx error: Display generic message like "Error processing request. Please check your input."
        -   If API returns 5xx error: Display generic message like "Server error. Please try again later."
        -   Network errors (fetch fails): Display generic message like "Network error. Could not reach server."

-   **Persistence**:
    -   The *last successful* `MutationResponse` object (containing `mutatedText` and the `mutations` array) is saved to browser `localStorage`.
    -   On page load, if this data exists in `localStorage`, the application rehydrates the state: displays the `mutatedText`, sets the total expected count for the counter, but *does not* automatically reveal/highlight mutations or restore the user's manual count.
    -   This persisted state is cleared when:
        -   A new mutation is successfully generated via the "Mutate" button.
        -   The user clicks "Reveal Mutations".

---

### 3. Technical Stack

-   **Frontend**: React (using functional components and hooks)
-   **Backend**: Rust (using a web framework like `axum` or `actix-web` for the HTTP server and JSON handling, integrating the existing `mutator.rs` logic)
-   **Hosting**: Azure App Service
-   **CI/CD**: GitHub Actions triggering builds and deployments. Bicep scripts for defining Azure infrastructure.
-   **Monitoring**: Basic OpenTelemetry integration in the Rust backend for request tracing/metrics (no complex dashboards planned for v1).

---

### 4. Testing

-   **Backend (Rust)**:
    -   Unit tests for `mutator.rs` logic.
    -   Unit tests for homophone logic (`homophones.rs`).
    -   Integration tests for the HTTP API endpoints (`/api/mutate`) covering success cases, input validation (text length), and basic error handling.
-   **Frontend (React)**:
    -   Jest/React Testing Library tests for key UI components (Input area, Settings panel, Display area, Buttons).
    -   Tests for core interactions: triggering mutation, incrementing counter, revealing mutations, copying text, displaying errors.

---

### 5. Deployment & DevOps

-   Single `main` branch (Trunk-Based Development).
-   Commits to `main` automatically trigger CI/CD pipeline via GitHub Actions.
-   Pipeline builds Rust backend, builds React frontend, packages them (e.g., into a Docker container), and deploys to Azure App Service using Bicep definitions.
-   Hosted using the default `*.azurewebsites.net` URL provided by App Service.

-   **Hosting Platform**: The application will be hosted on **Azure App Service**, specifically using the **Web App for Containers** service type.
-   **Rationale**: Azure App Service provides a robust Platform-as-a-Service (PaaS) environment suitable for web applications. As App Service does not have native build support for Rust applications, containerization is required.
-   **Containerization**: **Docker** will be used to package the application components into a single, deployable image. This image will contain:
    1.  The compiled **Rust backend** executable.
    2.  The static **React frontend** build artifacts (HTML, CSS, JavaScript files).
    3.  An **Nginx web server**.
-   **Container Runtime Configuration**:
    -   **Nginx** will serve as the primary entry point for incoming HTTP requests to the container (listening on the port specified by App Service, typically 80 or 8080).
    -   Nginx will be configured to:
        -   Serve the static React frontend files for requests to the root (`/`) and other non-API routes.
        -   Act as a reverse proxy for requests matching the `/api/*` path, forwarding them to the Rust backend service running within the same container (e.g., listening on `localhost:8080`).
    -   The **Rust backend application** will run as a separate process within the container, listening internally on a designated port (e.g., 8080) and handling only the API requests proxied by Nginx.
-   **Deployment Flow Integration**: The CI/CD pipeline (GitHub Actions) will be responsible for building the Docker image, pushing it to a container registry (like Azure Container Registry or Docker Hub), and triggering a deployment update on the Azure App Service instance.

---

This clearly outlines the chosen platform, the *why* (packaging Rust + React), the technology (Docker + Nginx), and the internal workings of the container, providing a solid plan for implementation.

---

### 6. Public Presence

-   Code hosted on a public GitHub repository.
-   README.md with basic project description and setup instructions.
-   No formal external documentation, contribution guides, or issue templates for v1.

---

### 7. Out of Scope for v1

-   Advanced accessibility features (WCAG compliance, screen reader optimization, dark mode, colorblind themes).
-   Dedicated mobile-first layout/fully responsive design (will rely on default browser/component behavior).
-   User accounts, authentication, saving results long-term.
-   Custom domain name.
-   Advanced analytics or logging beyond basic OTEL traces/metrics on the backend.
-   Click-to-identify interaction model.
-   Guaranteed mutation rate adherence.