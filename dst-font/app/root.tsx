import {
  Form,
  Links,
  Meta,
  NavLink,
  Outlet,
  Scripts,
  ScrollRestoration,
  json,
  useLoaderData,
} from "@remix-run/react";
import type { LinksFunction, LoaderFunctionArgs } from "@remix-run/node";
import appStylesHref from "./app.css?url";
import tailwindStylesHref from "./tailwind.css?url";
import i18next from "./conf/i18next.server";
import { useTranslation } from "react-i18next";
import { useChangeLanguage } from "remix-i18next/react";

export const links: LinksFunction = () => [
  { rel: "stylesheet", href: appStylesHref },
  { rel: "stylesheet", href: tailwindStylesHref },
];

export async function loader({ request }: LoaderFunctionArgs) {
  let locale = await i18next.getLocale(request);
  return json({ locale });
}

export default function App() {

  let { locale } = useLoaderData<typeof loader>();

  let { i18n, t } = useTranslation();

  // This hook will change the i18n instance language to the current locale
  // detected by the loader, this way, when we do something to change the
  // language, this locale will change and i18next will load the correct
  // translation files
  useChangeLanguage(locale);


  return (
    <html lang={locale} dir={i18n.dir()}>
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1"
        />
        <Meta />
        <Links />
      </head>
      <body>
        <div className="flex flex-col-reverse main">
          <div className="min-nav grid grid-cols-3 gap-3">
            <NavLink to="/games" className="min-nav-item text-center">
              {t('home-page')}
            </NavLink>
            <NavLink to="/import" className="min-nav-item text-center">
              {t('import-game')}
            </NavLink>
            <NavLink to="/setting" className="min-nav-item text-center">
              {t('setting')}
            </NavLink>
          </div>
          <Outlet />
        </div>
        <ScrollRestoration />
        <Scripts>
        </Scripts>
      </body>
    </html>
  );
}