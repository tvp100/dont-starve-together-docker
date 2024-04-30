import styleStylesHref from './style.css'
import type { LinksFunction, LoaderFunctionArgs } from "@remix-run/node";
import { redirect } from "@remix-run/node";


export const links: LinksFunction = () => [
  { rel: "stylesheet", href: styleStylesHref },
];

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const url = new URL(request.url);
  if (url.pathname === "/") {
    return redirect("/games");
  }
  return null;
};


export default function route() {
  return (
    <div></div>
  );
}