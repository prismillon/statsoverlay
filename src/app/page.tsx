import { Suspense } from "react";
import UserDataDisplay from "./display";

interface SearchParams {
  name?: string;
}

interface PageProps {
  searchParams: Promise<SearchParams>;
}

export default async function HomePage({ searchParams }: PageProps) {
  const params = await searchParams;
  if (!params.name) {
    return (
      <div className="p-8">
        <p>Please provide a name parameter (e.g., /?name=john)</p>
      </div>
    );
  }

  return (
    <main className="p-8">
      <Suspense fallback={<div>Loading...</div>}>
        <UserDataDisplay name={params.name} />
      </Suspense>
    </main>
  );
}
