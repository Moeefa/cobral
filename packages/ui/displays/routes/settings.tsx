import { ArrowLeftIcon } from "@radix-ui/react-icons";
import { Link } from "react-router-dom";

export function Settings() {
  return (
    <>
      <Link className="px-5 pt-4 flex items-center gap-1.5 w-min" to="/">
        <ArrowLeftIcon /> Voltar
      </Link>
    </>
  );
}
