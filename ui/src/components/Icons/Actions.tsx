import { FaSquare } from "react-icons/fa";
import { IoTriangleSharp } from "react-icons/io5";
import { BaseIcon } from "./BaseIcon";
import { FaSquareFull } from "react-icons/fa";

type ActionProps = {};

export const MajorAction = ({}: ActionProps) => {
  return (
    <BaseIcon size={22}>
      <FaSquareFull
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorAction = ({}: ActionProps) => {
  return (
    <BaseIcon size={20}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorActionLight = ({}: ActionProps) => {
  return (
    <BaseIcon size={20}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-6)",
        }}
      />
    </BaseIcon>
  );
};
