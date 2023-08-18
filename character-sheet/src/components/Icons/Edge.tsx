import { BiSolidStar } from "react-icons/bi";

type EdgeProps = {};

export const Edge = ({}: EdgeProps) => {
  return (
    <BiSolidStar
      size="18"
      style={{
        color: "var(--gray-9)",
      }}
    />
  );
};

export const EdgeLight = ({}: EdgeProps) => {
  return (
    <BiSolidStar
      size="14"
      style={{
        color: "var(--gray-6)",
      }}
    />
  );
};
