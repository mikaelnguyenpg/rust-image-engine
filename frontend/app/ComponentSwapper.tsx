"use client";

import dynamic from "next/dynamic";

const DynamicPhotoComponent = dynamic(() => import("./PhotoComponent"), {
  ssr: false,
  loading: () => <p>{"Loadding.."}</p>,
});

export default function ComponentSwapper() {
  return <DynamicPhotoComponent />;
}
