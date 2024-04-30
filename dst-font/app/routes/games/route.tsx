import { json, type LoaderFunctionArgs } from "@remix-run/node";
import GameCard from "./card";
import { useLoaderData } from "@remix-run/react";


const fake_data = [
  {
    title: '第一个时间',
    game_mod: null,
    description: '描述',
    password: '123456',
    player: '10',
    status: 'stop',
  },
  {
    title: '花花草草',
    game_mod: null,
    description: '描述',
    password: '123456',
    player: '10',
    status: 'stop',
  },
  {
    title: '木兰',
    game_mod: null,
    description: '描述',
    password: '123456',
    player: '10',
    status: 'stop',
  },
  {
    title: '油焖大虾',
    game_mod: null,
    description: '描述',
    password: '123456',
    player: '10',
    status: 'stop',
  },
  {
    title: '水仙',
    game_mod: null,
    description: '描述',
    password: '123456',
    player: '10',
    status: 'stop',
  },
];



export async function loader({ request }: LoaderFunctionArgs) {
  return json({ fake_data });
}


export default function games() {


  const { fake_data } = useLoaderData<typeof loader>();

  return (
    <>
      <div style={{ padding: '20px', overflow: 'auto' }}>
        {
        fake_data.map((item, index) => {
          return <GameCard {...item} key={index} />
        })
        }
      </div>
    </>
  );
}