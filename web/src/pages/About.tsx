function About() {
  return (
    <div className="prose lg:prose-xl mx-auto">
      <h1>About Feedtown</h1>
      <p>
        Feedtown is a modern feed aggregation platform built with:
      </p>
      <ul>
        <li>
          <strong>Backend:</strong> Rust with Axum for high-performance API
        </li>
        <li>
          <strong>Frontend:</strong> React with TypeScript for type safety
        </li>
        <li>
          <strong>Runtime:</strong> Deno for secure JavaScript execution
        </li>
        <li>
          <strong>Styling:</strong> Tailwind CSS with DaisyUI components
        </li>
        <li>
          <strong>Routing:</strong> React Router for seamless navigation
        </li>
        <li>
          <strong>Build Tool:</strong> Vite for fast development and building
        </li>
      </ul>
      <div className="card w-full bg-base-200 shadow-xl mt-8">
        <div className="card-body">
          <h2 className="card-title">Features</h2>
          <p>
            Connect with the API running on port 3000 to manage your feeds and
            content.
          </p>
          <div className="card-actions justify-end">
            <div className="badge badge-outline">API Integration</div>
            <div className="badge badge-outline">Real-time Updates</div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default About;
