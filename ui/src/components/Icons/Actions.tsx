import { FaSquare } from "react-icons/fa";
import { BsTriangleFill } from "react-icons/bs";

type ActionProps = {};

export const MajorAction = ({}: ActionProps) => {
  return (
    <FaSquare
      size="18"
      style={{
        color: "var(--gray-9)",
      }}
    />
  );
};

export const MinorAction = ({}: ActionProps) => {
  return (
    <BsTriangleFill
      size="14"
      style={{
        color: "var(--gray-9)",
      }}
    />
  );
};

export const MinorActionLight = ({}: ActionProps) => {
  return (
    <BsTriangleFill
      size="14"
      style={{
        color: "var(--gray-6)",
      }}
    />
  );
};
