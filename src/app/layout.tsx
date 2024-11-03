export const metadata = {
  title: "lounge overlay",
  description: "lounge 150cc stats overlay by prismillon",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
